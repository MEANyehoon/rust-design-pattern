pub trait Pizza {
    fn get_name(&self) -> &str;
    fn prepare(&self);
    fn bake(&self);
    fn cut(&self);
}