use super::super::material::node;
struct ModelManager<'a>{
    mngNodes: &'a Vec<node::Node>,
}

trait AddModel<T>{
    fn add_model(&mut self, c:T);
}

impl AddModel<node::Node> for ModelManager{
    fn add_model(&mut self, c: node::Node){
        self.mngNodes.push(c);
    }
}