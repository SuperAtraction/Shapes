pub trait Shape {
    fn area(&self) -> f64;
    fn info(&self) -> String;
    fn id(&self) -> i32;

    fn deleted_message(&self) -> String {
        format!("Suppression : #{} - {}", self.id(), self.info())
    }
}
