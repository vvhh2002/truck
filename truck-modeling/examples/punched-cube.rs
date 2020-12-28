//! Modeling a unit cube with a hole through it.

mod framework;
use framework::ShapeViewer;
use truck_rendimpl::*;

fn main() {
    let v = builder::vertex(Point3::new(-0.5, -0.5, -0.5));
    let edge = builder::tsweep(&v, Vector3::unit_x());
    let mut face = builder::tsweep(&edge, Vector3::unit_y());
    let v = builder::vertex(Point3::new(0.2, 0.0, -0.5));
    let edge0 = builder::tsweep(&v, Vector3::new(-0.2, 0.2, 0.0));
    let edge1 = builder::rsweep(
        edge0.back(),
        Point3::origin(),
        Vector3::unit_z(),
        Rad(std::f64::consts::PI / 2.0),
    )
    .pop_back()
    .unwrap();
    let edge2 = builder::tsweep(edge1.back(), Vector3::new(0.2, -0.2, 0.0));
    let edge3 = builder::rsweep(
        edge2.back(),
        Point3::origin(),
        Vector3::unit_z(),
        Rad(std::f64::consts::PI / 2.0),
    )
    .pop_back()
    .unwrap();
    let edge3 = Edge::new(
        edge3.front(),
        edge0.front(),
        edge3.lock_curve().unwrap().clone(),
    );
    let wire = Wire::from(vec![
        edge3.inverse(),
        edge2.inverse(),
        edge1.inverse(),
        edge0.inverse(),
    ]);
    face.add_boundary(wire);
    let shape = builder::tsweep(&face, Vector3::unit_z());
    ShapeViewer::run(shape);
}