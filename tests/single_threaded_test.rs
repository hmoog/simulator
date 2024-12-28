use simulator::{define_thread_safe_node, Network};

#[derive(Debug, Clone)]
enum PingPongProtocol { Ping, Pong }

// Node with a counter that increments on each received message.
define_thread_safe_node!(SomeNode, PingPongProtocol, {
    counter: usize,
}, {
    run: |node| {
        node.network().send(1, PingPongProtocol::Ping, 0);
    },
    handle_message: |node, message, ctx| {
        node.counter = node.counter + 1;

        match message {
            PingPongProtocol::Pong => println!(
                "Node {} received message from {}: {:?}",
                ctx.to(),
                ctx.from(),
                message
            ),
            _ => {}
        }
    },
});

// Node that sends a message to another node and waits for a reply.
define_thread_safe_node!(PingNode, PingPongProtocol, {
    run: |node| {
        node.network().send(1, PingPongProtocol::Ping, 0);
    },
    handle_message: |_node, message, ctx| {
        match message {
            PingPongProtocol::Pong => println!(
                "Node {} received message from {}: {:?}",
                ctx.to(),
                ctx.from(),
                message
            ),
            _ => {}
        }
    },
});

// Node that replies to a message with another message with some processing delays.
define_thread_safe_node!(PongNode, PingPongProtocol, {
    run: |_| {},
    handle_message: |_node, message, ctx| {
        if let PingPongProtocol::Ping = message {
            println!("Node {} received message from {}: {:?}", ctx.to(), ctx.from(), message);

            ctx.reply(PingPongProtocol::Pong, 0);

            ctx.schedule(3, {
                let ctx = ctx.clone();
                move || {
                    ctx.reply(PingPongProtocol::Pong, 0);

                    println!("Node replied later to message from {}", ctx.from());
                }
            });

            println!("done");
        }
    },
});

#[test]
fn test_simulator() {
    let mut network = Network::new();

    network.add_node(PingNode::new());
    network.add_node(PongNode::new());

    network.run()
}