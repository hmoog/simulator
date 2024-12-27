#[macro_export]
macro_rules! define_node {
    ($node_name:ident, $protocol:ty,
     { $($data_field:ident: $data_type:ty,)* },
     {
        run: $run_body:expr,
        handle_message: $receive_body:expr,
     }) => {
        struct $node_name {
            network_adapter: Option<$crate::NetworkAdapter<$protocol>>,
            $($data_field: $data_type,)*
        }

        impl $node_name {
            fn new($($data_field: $data_type,)*) -> Self {
                Self {
                    network_adapter: None,
                    $($data_field,)*
                }
            }

            fn network(&self) -> &$crate::NetworkAdapter<$protocol> {
                self.network_adapter.as_ref().expect("not initialized")
            }
        }

        impl $crate::NodeInterface<$protocol> for $node_name {
            fn start(&mut self, adapter: $crate::NetworkAdapter<$protocol>) {
                self.network_adapter = Some(adapter);
            }

            fn run(&mut self) {
                let run_fn: fn(&mut $node_name) = $run_body;
                run_fn(self);
            }

            fn receive_message(
                &mut self,
                message: $protocol,
                ctx: $crate::MessageContext<$protocol>,
            ) {
                let receive_fn: fn(&mut $node_name, $protocol, $crate::MessageContext<$protocol>) = $receive_body;
                receive_fn(self, message, ctx);
            }
        }
    };

    ($node_name:ident, $protocol:ty,
     {
        run: $run_body:expr,
        handle_message: $receive_body:expr,
     }) => {
        struct $node_name {
            network_adapter: Option<$crate::NetworkAdapter<$protocol>>,
        }

        impl $node_name {
            fn new() -> Self {
                Self {
                    network_adapter: None,
                }
            }

            fn network(&self) -> &$crate::NetworkAdapter<$protocol> {
                self.network_adapter.as_ref().expect("not initialized")
            }
        }

        impl $crate::NodeInterface<$protocol> for $node_name {
            fn start(&mut self, adapter: $crate::NetworkAdapter<$protocol>) {
                self.network_adapter = Some(adapter);
            }

            fn run(&mut self) {
                let run_fn: fn(&mut $node_name) = $run_body;
                run_fn(self);
            }

            fn receive_message(
                &mut self,
                message: $protocol,
                ctx: $crate::MessageContext<$protocol>,
            ) {
                let receive_fn: fn(&mut $node_name, $protocol, $crate::MessageContext<$protocol>) = $receive_body;
                receive_fn(self, message, ctx);
            }
        }
    };
}

#[macro_export]
macro_rules! define_thread_safe_node {
    ($node_name:ident, $protocol:ty,
     { $($data_field:ident: $data_type:ty,)* },
     {
        run: $run_body:expr,
        handle_message: $receive_body:expr,
     }) => {
        $crate::define_node!(
            $node_name, $protocol,
            {
                $($data_field: $data_type,)*
            },
            {
                run: $run_body,
                handle_message: {
                    fn thread_safe_handle_message(
                        node: &mut $node_name,
                        message: $protocol,
                        ctx: $crate::MessageContext<$protocol>
                    ) {
                        println!("[ START ] Message From {} To {}", ctx.from(), ctx.to());
                        // TODO: set internal flag to indicate that the message is being processed
                        // if flag is set -> queue

                        let my_ctx = $crate::single_threaded::MessageContext::new(ctx, {
                            move |ctx| {
                                println!("[ DONE ] Message From {} To {}", ctx.from(), ctx.to());
                                // TODO: clear internal flag
                                // check if there are queued messages and process them
                            }
                        });

                        let receive_fn: fn(&mut $node_name, $protocol, $crate::single_threaded::MessageContext<$protocol>) = $receive_body;
                        receive_fn(node, message, my_ctx);
                    }

                    thread_safe_handle_message as fn(&mut $node_name, $protocol, $crate::MessageContext<$protocol>)
                },
            }
        );
    };

    ($node_name:ident, $protocol:ty,
     {
        run: $run_body:expr,
        handle_message: $receive_body:expr,
     }) => {
        $crate::define_node!($node_name, $protocol, {
            run: $run_body,
            handle_message: {
                fn thread_safe_handle_message(
                    node: &mut $node_name,
                    message: $protocol,
                    ctx: $crate::MessageContext<$protocol>
                ) {
                    println!("[ START ] Message From {} To {}", ctx.from(), ctx.to());
                    // TODO: set internal flag to indicate that the message is being processed
                    // if flag is set -> queue

                    let my_ctx = $crate::single_threaded::MessageContext::new(ctx, {
                        move |ctx| {
                            println!("[ DONE ] Message From {} To {}", ctx.from(), ctx.to());
                            // TODO: clear internal flag
                            // check if there are queued messages and process them
                        }
                    });

                    let receive_fn: fn(&mut $node_name, $protocol, $crate::single_threaded::MessageContext<$protocol>) = $receive_body;
                    receive_fn(node, message, my_ctx);
                }

                thread_safe_handle_message as fn(&mut $node_name, $protocol, $crate::MessageContext<$protocol>)
            },
        });
    };
}