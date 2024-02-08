use crate::render::render_engine::RenderEngine;

/// A trait defining the standard functions for a component to be used by the engine.
/// 
/// T - The other components needed from the engine to use this component.
pub trait EngineComponent<T> {
    /// Creates a new instance of this component using a mutable reference to a `RenderEngine`.
    fn create(engine: &mut RenderEngine) -> Self;

    /// Called when the engine starts all the components after they are all created.
    fn start(&mut self, other: T);

    /// Called when the engine updates.
    fn update(&mut self, other: T);

    /// Called when this component is called to render during its render pass defined in the created `App`.
    fn render<'rpass>(&'rpass mut self, render_engine: &'rpass RenderEngine, pass: &mut wgpu::RenderPass<'rpass>);

    /// Called when the engine exits.
    fn exit(&mut self, other: T);
}

/// This macro creates a `App` objects using a given set of components and some render pass descriptions.
/// 
/// Example:
/// ```rust
/// create_app!(
///    // The color used when the display clears before drawing the next frame.
///    CLEAR_COLOR = wgpu::Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 },
///
///    // A list of components and the fields needed to call them (see the generic argument in `EngineComponent` for more info).
///    APP {
///        ui_engine: UIEngine[render_engine],
///        test: TestComponent[render_engine, ui_engine]
///    },
///
///    // List of render passes to draw.  They will be rendered in order with the clear color applied to whichever is marked as 0.
///    // The COMPONENTS array is a list of fields that must be a subset of the fields defined in the APP section and their render functions will be called in order.
///    // The DEPTH boolean defines if a depth texture should be present in the render pass used.
///    PASSES {
///        0: {
///            COMPONENTS: [test],
///            DEPTH: true
///        },
///        1: {
///            COMPONENTS: [ui_engine],
///            DEPTH: false
///        }
///    }
///);
/// ```
#[macro_export]
macro_rules! create_app {
    (
        CLEAR_COLOR = $color:expr,
        APP {$(
            $component:ident: $type:ty[$($param:ident),*]
        ),*},
        PASSES {$(
            $pass_idx:literal: {
                COMPONENTS: [$($to_render:ident),*],
                DEPTH: $depth:literal
            }
        ),*}
    ) => {
        use forte_engine::{EngineApp, start_render, end_render, pass, inputs::{Inputs, winit_input::EngineInput}, render::{render_engine::RenderEngine, render_utils}};

        pub struct App {
            render_engine: RenderEngine,
            inputs: Inputs,
            $($component: $type,)*
        }

        impl EngineApp for App {
            // Takes in a render engine and creates each component individually in the order listed, then saves them into a new instance of App.
            fn create(mut render_engine: RenderEngine) -> Self {
                let inputs = Inputs::new();
                $(let $component = <$type>::create(&mut render_engine);)*
                Self {
                    render_engine,
                    inputs,
                    $($component,)*
                }
            }

            // Starts app components of the App.
            fn start(&mut self) {
                $(
                    <$type>::start(&mut self.$component, ($(&mut self.$param),*));
                )*
            }

            // Updates App components, then performs the render steps in the order given.
            fn update(&mut self) {
                // Run update
                $(
                    <$type>::update(&mut self.$component, ($(&mut self.$param),*));
                )*

                // start the render
                let resources = render_utils::prepare_render(&self.render_engine);
                let mut resources = if resources.is_ok() { resources.unwrap() } else { return };

                // run each render pass in the order given
                $(
                    {
                        // create color attachment for this pass
                        let pass_id = $pass_idx;
                        let color_attachment = wgpu::RenderPassColorAttachment {
                            view: &resources.view,
                            resolve_target: None,
                            ops: if pass_id == 0 {
                                wgpu::Operations {
                                    load: wgpu::LoadOp::Clear($color),
                                    store: wgpu::StoreOp::Store,
                                }
                            } else {
                                wgpu::Operations {
                                    load: wgpu::LoadOp::Load,
                                    store: wgpu::StoreOp::Store,
                                }
                            },
                        };

                        // create the render pass
                        let mut pass = resources.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                            label: Some("Render Pass"),
                            color_attachments: &[Some(color_attachment)],
                            depth_stencil_attachment: 
                                if !$depth { None } 
                                else { 
                                    Some(wgpu::RenderPassDepthStencilAttachment {
                                        view: &self.render_engine.depth_texture.view,
                                        depth_ops: Some(wgpu::Operations {
                                            load: wgpu::LoadOp::Clear(1.0),
                                            store: wgpu::StoreOp::Store
                                        }),
                                        stencil_ops: None
                                    })
                                },
                            occlusion_query_set: None,
                            timestamp_writes: None,
                        });

                        // call all members of this pass' render functions
                        $(
                            self.$to_render.render(&self.render_engine, &mut pass);
                        )*
                    }
                )*

                // end the render
                render_utils::finalize_render(&mut self.render_engine, resources);

                // call next frame, will be replaced later
                self.render_engine.next_frame();

                // reset inputs
                self.inputs.reset();
            }

            // takes all input from the event loop, will be processed later
            fn input(&mut self, input: EngineInput) {
                self.inputs.handle_input(input);
            }
            
            // passes all resize from the event loop to the render engine
            fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) { self.render_engine.resize(new_size); }

            // calls all the exit functions of the components in the order given
            fn exit(&mut self) {
                $(
                    <$type>::exit(&mut self.$component, ($(&mut self.$param),*));
                )*
            }
        }
    };
}
