use simulator::{define_node, Network};

#[derive(Debug, Clone)]
enum PingPongProtocol { Ping, Pong }

define_node!(PingNode, PingPongProtocol, {
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

define_node!(PongNode, PingPongProtocol, {
    run: |_| {},
    handle_message: |_node, message, ctx| {
        if let PingPongProtocol::Ping = message {
            ctx.reply(PingPongProtocol::Pong, 0);
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