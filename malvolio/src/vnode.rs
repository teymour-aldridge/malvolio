use yew::virtual_dom::VNode;

pub trait IntoVNode {
    fn into_vnode(self) -> VNode;
}
