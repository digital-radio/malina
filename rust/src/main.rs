use dbus::blocking::Connection;
use std::time::Duration;
use dbus_crossroads::{Crossroads, Context};
use std::env;
use std::error::Error;

struct Hello { called_count: u32 }

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("usage: ./malina client|server");
        return;
    }

    if &args[1] == "client" {
        let result = client();

        match result {
            Ok(_) => println!("Client done"),
            Err(e) => panic!("Problem with running a client: {:?}", e),
        };
    } else {
        let result = server();

        match result {
            Ok(_) => println!("Server done"),
            Err(e) => panic!("Problem with running a client: {:?}", e),
        };
    }
}

fn client() -> Result<(), Box<dyn Error>> {
    // First open up a connection to the session bus.
    let conn = Connection::new_session()?;

    // Second, create a wrapper struct around the connection that makes it easy
    // to send method calls to a specific destination and path.
    let proxy = conn.with_proxy("com.example.dbustest", "/hello", Duration::from_millis(5000));

    // Now make the method call. The ListNames method call takes zero input parameters and
    // one output parameter which is an array of strings.
    // Therefore the input is a zero tuple "()", and the output is a single tuple "(names,)".
    let (text,): (String,) = proxy.method_call("com.example.dbustest", "Hello", ("michael",))?;

    // Let's print all the names to stdout.
    println!("{}", text);

    Ok(())
}

fn server() -> Result<(), Box<dyn Error>> {
    let c = Connection::new_session()?;
    c.request_name("com.example.dbustest", false, true, false)?;

    let mut cr = Crossroads::new();

    // Let's build a new interface, which can be used for "Hello" objects.
    let iface_token = cr.register("com.example.dbustest", |b| {
        // This row is just for introspection: It advertises that we can send a
        // HelloHappened signal. We use the single-tuple to say that we have one single argument,
        // named "sender" of type "String".
        b.signal::<(String,), _>("HelloHappened", ("sender",));
        // Let's add a method to the interface. We have the method name, followed by
        // names of input and output arguments (used for introspection). The closure then controls
        // the types of these arguments. The last argument to the closure is a tuple of the input arguments.
        b.method("Hello", ("name",), ("reply",), |ctx: &mut Context, hello: &mut Hello, (name,): (String,)| {
            // And here's what happens when the method is called.
            println!("Incoming hello call from {}!", name);
            hello.called_count += 1;
            let s = format!("Hello {}! This API has been used {} times.", name, hello.called_count);
            // The ctx parameter can be used to conveniently send extra messages.
            let signal_msg = ctx.make_signal("HelloHappened", (name,));
            ctx.push_msg(signal_msg);
            // And the return value is a tuple of the output arguments.
            Ok((s,))
        });
    });

    // Let's add the "/hello" path, which implements the com.example.dbustest interface,
    // to the crossroads instance.
    cr.insert("/hello", &[iface_token], Hello { called_count: 0});

    // Serve clients forever.
    cr.serve(&c)?;

    unreachable!()
}
