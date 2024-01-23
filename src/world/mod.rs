pub mod app;
pub mod dimensions;

/// Generates a component definition with its ComponentDef supporting functions and render functions.
/// 
/// Example:
/// ```rust 
/// define_components!(
/// Components,
/// TestApp,
/// DrawNodes,
/// draw_node,
/// [
///     CubeModel => {
///         DATA => CubeModel,
///         ADDED => |_: &mut TestApp, _: &mut Node| { println!("Added"); },
///         UPDATE => |_: &mut TestApp, _: &mut Node| { println!("Updated"); },
///         RENDER => |pass: &mut wgpu::RenderPass<'a>, app: &'b TestApp, data: &'b CubeModel| {
///             pass.prepare_cube_engine(&app.cube_engine, &app.camera);
///             pass.draw_cube_model(&app.render_engine, &app.cube_engine, data);
///         },
///         REMOVED => |_: &mut TestApp, _: &mut Node| { println!("Removed"); }
///     }
/// ]
/// );
/// ```
#[macro_export]
macro_rules! run_world {

    (
        $app:ty,
        [$(
            $variant:ident => {
                DATA => $data:ty,
                ADDED => $added:expr,
                UPDATE => $update:expr,
                RENDER => $render:expr,
                REMOVED => $removed:expr
            }
        ),*]
    ) => {
        use std::marker::PhantomData;
        use cgmath::{Vector3, ElementWise};
        use forte_engine::{math::transforms::Transform, world::dimensions::Dimensions, EngineApp, start_render, end_render, pass, run_app};

        // Create full enum
        #[derive(Default, Debug)]
        pub enum Component {
            #[default]
            Empty,
            $($variant($data),)*
        }

        // create node
        #[derive(Debug)]
        pub struct Node {
            // public
            pub transform: Transform,
            pub component: Component,
            pub rel_min_dimensions: Dimensions,
        
            // non-public
            global_transform: Transform,
            dimensions: Dimensions,
            children: Vec<Node>
        }

        // Give node a default
        impl Default for Node {
            fn default() -> Self {
                Self {
                    transform: Transform::default(),
                    global_transform: Transform::default(),
                    rel_min_dimensions: Dimensions::default(),
                    dimensions: Dimensions::default(),
                    component: Component::default(),
                    children: Vec::new()
                }
            }
        }

        // create node functions
        impl Node {
            // accessor functions
            pub fn global_transform(&self) -> &Transform { &self.transform }
            pub fn rel_min_dimensions(&self) -> &Dimensions { &self.rel_min_dimensions }
            pub fn dimensions(&self) -> &Dimensions { &self.dimensions }
            pub fn children(&self) -> &Vec<Node> { &self.children }

            // modification functions
            pub fn add_child(&mut self, app: &mut $app, mut child: Node) {
                self.children.push(child);
                self.children.last_mut().as_mut().unwrap().call_add_recr(app);
            }

            pub fn remove_child(&mut self, app: &mut $app, idx: usize) {
                self.children[idx].call_remove_recr(app);
                self.children.remove(idx);
            }

            pub fn update(&mut self, app: &mut $app, previous: &Transform) {
                // calculate new global transform
                let global_transform = Transform {
                    position: self.transform.position + previous.position,
                    rotation: previous.rotation * self.transform.rotation,
                    scale: self.transform.scale.mul_element_wise(previous.scale)
                };

                // calculate starting dimensions
                let mut dimensions = Dimensions {
                    from: Vector3 {
                        x: global_transform.position.x + self.rel_min_dimensions.from.x,
                        y: global_transform.position.y + self.rel_min_dimensions.from.y,
                        z: global_transform.position.z + self.rel_min_dimensions.from.z,
                    },
                    to: Vector3 {
                        x: global_transform.position.x + self.rel_min_dimensions.to.x,
                        y: global_transform.position.y + self.rel_min_dimensions.to.y,
                        z: global_transform.position.z + self.rel_min_dimensions.to.z,
                    }
                };

                // update children first, and update dimensions if/when necessary
                self.children.iter_mut().for_each(|child| {
                    child.update(app, &global_transform);

                    // check for dimension updates
                    if child.dimensions.from.x < dimensions.from.x { dimensions.from.x = child.dimensions.from.x; }
                    if child.dimensions.from.y < dimensions.from.y { dimensions.from.y = child.dimensions.from.y; }
                    if child.dimensions.from.z < dimensions.from.z { dimensions.from.z = child.dimensions.from.z; }
                    if child.dimensions.to.x > dimensions.to.x { dimensions.to.x = child.dimensions.to.x; }
                    if child.dimensions.to.y > dimensions.to.y { dimensions.to.y = child.dimensions.to.y; }
                    if child.dimensions.to.z > dimensions.to.z { dimensions.to.z = child.dimensions.to.z; }
                });

                // update global transform and dimensions
                self.global_transform = global_transform;
                self.dimensions = dimensions;

                // call component update
                match &self.component {
                    Component::Empty => {},
                    $(Component::$variant(data) => { $update(app, self) },)*
                }
            }

            // calls the add functions recursively for this node and all its children
            pub(crate) fn call_add_recr(&mut self, app: &mut $app) {
                match &self.component {
                    Component::Empty => {},
                    $(Component::$variant(data) => { $added(app, self) },)*
                }

                self.children.iter_mut().for_each(|child| child.call_add_recr(app));
            }

            // calls the remove functions recursively for this node and all its children
            pub(crate) fn call_remove_recr(&mut self, app: &mut $app) {
                match &self.component {
                    Component::Empty => {},
                    $(Component::$variant(data) => { $removed(app, self) },)*
                }

                self.children.iter_mut().for_each(|child| child.call_remove_recr(app));
            }
        }
    
        // create render trait
        pub trait DrawNodes <'a,'b> where 'b: 'a {
            fn draw_node(
                &mut self,
                app: &'b $app,
                node: &'b Node
            );
        }

        // draw trait for render pass
        impl<'a, 'b> DrawNodes <'a, 'b> for wgpu::RenderPass<'a> where 'b: 'a {
            fn draw_node(
                &mut self,
                app: &'b $app,
                node: &'b Node
            ) {
                match &node.component {
                    Component::Empty => {},
                    $(Component::$variant(data) => { $render(self, app, data) },)*
                }

                node.children().iter().for_each(|child| self.draw_node(app, child));
            }
        }

        pub trait WorldApp {
            fn render_engine(&self) -> &RenderEngine;
            fn render_engine_mut(&mut self) -> &mut RenderEngine;

            fn create(render_engine: RenderEngine) -> Self;
            fn start(&mut self, root: &mut Node);
            fn update(&mut self, root: &mut Node);
            fn exit(&mut self, root: &mut Node);
        }
        
        pub struct WorldContainer {
            pub app: $app,
            pub root: Node
        }

        // add an engine app so that we can run the world container as an app
        impl EngineApp for WorldContainer {
            // Simply create a new root node and create the above world app
            fn create(mut engine: RenderEngine) -> Self {
                // create app and root node
                let mut app = <$app>::create(engine);
                let mut root = Node::default();

                // start the app
                app.start(&mut root);

                // create new world container and return
                Self { app, root }
            }

            // Update the app, then its nodes
            fn update(&mut self) {
                // call updates
                self.app.update(&mut self.root);
                self.root.update(&mut self.app, &Transform::default());

                // do render
                let mut resources = start_render!(self.app.render_engine_mut());
                {
                    let mut pass = pass!(self.app.render_engine(), resources);
                    pass.draw_node(&self.app, &self.root);
                }
                end_render!(self.app.render_engine_mut(), resources);
            }

            fn input(&mut self, input: forte_engine::render::input::EngineInput) {}
            fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) { self.app.render_engine_mut().resize(new_size); }
            fn events_cleared(&mut self) { self.app.render_engine_mut().next_frame(); }
            fn exit(&mut self) { self.app.exit(&mut self.root); }
        }

        // run everything we just created
        fn main() { run_app::<WorldContainer>(); }
    };
}
