use wavefront_obj::mtl::MtlSet;
use wavefront_obj::obj::{self, ObjSet};
use std::sync::Arc;
use current::Current;
use dyon::*;
use dyon::embed::{PushVariable, PopVariable};

pub type Materials = Vec<(Arc<String>, MtlSet)>;
pub type ObjSets = Vec<(Arc<String>, ObjSet)>;

pub fn register_obj(module: &mut Module) {
    module.add(Arc::new("load_material".into()),
        load_material, PreludeFunction {
            lts: vec![Lt::Default],
            tys: vec![Type::Text],
            ret: Type::Result(Box::new(Type::Text))
        });
    module.add(Arc::new("material".into()),
        material, PreludeFunction {
            lts: vec![Lt::Default],
            tys: vec![Type::Text],
            ret: Type::Option(Box::new(Type::F64))
        });
    module.add(Arc::new("materials".into()),
        materials, PreludeFunction {
            lts: vec![],
            tys: vec![],
            ret: Type::Array(Box::new(Type::Text))
        });
    module.add(Arc::new("load_obj".into()),
        load_obj, PreludeFunction {
            lts: vec![Lt::Default],
            tys: vec![Type::Text],
            ret: Type::Result(Box::new(Type::Text))
        });
    module.add(Arc::new("obj".into()),
        obj, PreludeFunction {
            lts: vec![Lt::Default],
            tys: vec![Type::Text],
            ret: Type::Option(Box::new(Type::F64))
        });
    module.add(Arc::new("objs".into()),
        objs, PreludeFunction {
            lts: vec![],
            tys: vec![],
            ret: Type::Array(Box::new(Type::Text))
        });
    module.add(Arc::new("material_library_obj".into()),
        material_library_obj, PreludeFunction {
            lts: vec![Lt::Default],
            tys: vec![Type::F64],
            ret: Type::Option(Box::new(Type::Text))
        });
    module.add(Arc::new("object_count_obj".into()),
        object_count_obj, PreludeFunction {
            lts: vec![Lt::Default],
            tys: vec![Type::F64],
            ret: Type::F64
        });
    module.add(Arc::new("objects_obj".into()),
        objects_obj, PreludeFunction {
            lts: vec![Lt::Default],
            tys: vec![Type::F64],
            ret: Type::Array(Box::new(Type::Text))
        });
    module.add(Arc::new("vertex_count_obj_object".into()),
        vertex_count_obj_object, PreludeFunction {
            lts: vec![Lt::Default; 2],
            tys: vec![Type::F64; 2],
            ret: Type::F64
        });
    module.add(Arc::new("tex_vertex_count_obj_object".into()),
        tex_vertex_count_obj_object, PreludeFunction {
            lts: vec![Lt::Default; 2],
            tys: vec![Type::F64; 2],
            ret: Type::F64
        });
    module.add(Arc::new("normal_count_obj_object".into()),
        normal_count_obj_object, PreludeFunction {
            lts: vec![Lt::Default; 2],
            tys: vec![Type::F64; 2],
            ret: Type::F64
        });
    module.add(Arc::new("geometry_count_obj_object".into()),
        geometry_count_obj_object, PreludeFunction {
            lts: vec![Lt::Default; 2],
            tys: vec![Type::F64; 2],
            ret: Type::F64
        });
    module.add(Arc::new("vertex_obj_object_vertex".into()),
        vertex_obj_object_vertex, PreludeFunction {
            lts: vec![Lt::Default; 3],
            tys: vec![Type::F64; 3],
            ret: Type::Vec4
        });
    module.add(Arc::new("tex_vertex_obj_object_tex_vertex".into()),
        tex_vertex_obj_object_tex_vertex, PreludeFunction {
            lts: vec![Lt::Default; 3],
            tys: vec![Type::F64; 3],
            ret: Type::Vec4
        });
    module.add(Arc::new("normal_obj_object_normal".into()),
        normal_obj_object_normal, PreludeFunction {
            lts: vec![Lt::Default; 3],
            tys: vec![Type::F64; 3],
            ret: Type::Vec4
        });
    module.add(Arc::new("vertices_obj_object".into()),
        vertices_obj_object, PreludeFunction {
            lts: vec![Lt::Default; 2],
            tys: vec![Type::F64; 2],
            ret: Type::Array(Box::new(Type::Vec4))
        });
    module.add(Arc::new("tex_vertices_obj_object".into()),
        tex_vertices_obj_object, PreludeFunction {
            lts: vec![Lt::Default; 2],
            tys: vec![Type::F64; 2],
            ret: Type::Array(Box::new(Type::Vec4))
        });
    module.add(Arc::new("normals_obj_object".into()),
        normals_obj_object, PreludeFunction {
            lts: vec![Lt::Default; 2],
            tys: vec![Type::F64; 2],
            ret: Type::Array(Box::new(Type::Vec4))
        });
    module.add(Arc::new("geometry_obj_object".into()),
        geometry_obj_object, PreludeFunction {
            lts: vec![Lt::Default; 2],
            tys: vec![Type::F64; 2],
            ret: Type::Array(Box::new(Type::object()))
        });
}

dyon_fn!{fn load_material(file: Arc<String>) -> Result<Arc<String>, String> {
    use wavefront_obj::mtl::parse;
    use std::fs::File;
    use std::io::Read;
    use std::error::Error;

    let materials = unsafe { &mut *Current::<Materials>::new() };

    let mut f = try!(File::open(&**file).map_err(|err| String::from(err.description())));
    let mut s = String::new();
    try!(f.read_to_string(&mut s).map_err(|err| String::from(err.description())));

    let mtlset = try!(parse(s).map_err(|err|
        format!("Error when parsing `{}`:\n{}:{}", file, err.line_number, err.message)));
    materials.push((file.clone(), mtlset));
    Ok(file)
}}

dyon_fn!{fn load_obj(file: Arc<String>) -> Result<Arc<String>, String> {
    use wavefront_obj::obj::parse;
    use std::fs::File;
    use std::io::Read;
    use std::error::Error;

    let obj_sets = unsafe { &mut *Current::<ObjSets>::new() };

    let mut f = try!(File::open(&**file).map_err(|err| String::from(err.description())));
    let mut s = String::new();
    try!(f.read_to_string(&mut s).map_err(|err| String::from(err.description())));

    let obj_set = try!(parse(s).map_err(|err|
        format!("Error when parsing `{}`:\n{}:{}", file, err.line_number, err.message)));
    obj_sets.push((file.clone(), obj_set));
    Ok(file)
}}

dyon_fn!{fn material(file: Arc<String>) -> Option<usize> {
    let materials = unsafe { &*Current::<Materials>::new() };
    for (i, mat) in materials.iter().enumerate() {
        if &mat.0 == &file { return Some(i); }
    }
    None
}}

dyon_fn!{fn obj(file: Arc<String>) -> Option<usize> {
    let obj_sets = unsafe { &*Current::<ObjSets>::new() };
    for (i, obj_set) in obj_sets.iter().enumerate() {
        if &obj_set.0 == &file { return Some(i); }
    }
    None
}}

dyon_fn!{fn materials() -> Vec<Arc<String>> {
    let materials = unsafe { &*Current::<Materials>::new() };
    materials.iter().map(|n| n.0.clone()).collect()
}}

dyon_fn!{fn objs() -> Vec<Arc<String>> {
    let obj_sets = unsafe { &*Current::<ObjSets>::new() };
    obj_sets.iter().map(|n| n.0.clone()).collect()
}}

dyon_fn!{fn material_library_obj(ind: usize) -> Option<Arc<String>> {
    let obj_sets = unsafe { &*Current::<ObjSets>::new() };
    obj_sets[ind].1.material_library.as_ref().map(|n| Arc::new(n.clone()))
}}

dyon_fn!{fn object_count_obj(ind: usize) -> usize {
    let obj_sets = unsafe { &*Current::<ObjSets>::new() };
    obj_sets[ind].1.objects.len()
}}

dyon_fn!{fn objects_obj(ind: usize) -> Vec<Arc<String>> {
    let obj_sets = unsafe { &*Current::<ObjSets>::new() };
    obj_sets[ind].1.objects.iter().map(|n| Arc::new(n.name.clone())).collect()
}}

dyon_fn!{fn vertex_count_obj_object(obj: usize, object: usize) -> usize {
    let obj_sets = unsafe { &*Current::<ObjSets>::new() };
    obj_sets[obj].1.objects[object].vertices.len()
}}

dyon_fn!{fn tex_vertex_count_obj_object(obj: usize, object: usize) -> usize {
    let obj_sets = unsafe { &*Current::<ObjSets>::new() };
    obj_sets[obj].1.objects[object].tex_vertices.len()
}}

dyon_fn!{fn normal_count_obj_object(obj: usize, object: usize) -> usize {
    let obj_sets = unsafe { &*Current::<ObjSets>::new() };
    obj_sets[obj].1.objects[object].normals.len()
}}

dyon_fn!{fn geometry_count_obj_object(obj: usize, object: usize) -> usize {
    let obj_sets = unsafe { &*Current::<ObjSets>::new() };
    obj_sets[obj].1.objects[object].geometry.len()
}}

dyon_fn!{fn vertex_obj_object_vertex
    (obj: usize, object: usize, vertex: usize) -> Vec4 {
    let obj_sets = unsafe { &*Current::<ObjSets>::new() };
    let vertex = obj_sets[obj].1.objects[object].vertices[vertex];
    [vertex.x, vertex.y, vertex.z].into()
}}

dyon_fn!{fn tex_vertex_obj_object_tex_vertex
    (obj: usize, object: usize, tex_vertex: usize) -> Vec4 {
    let obj_sets = unsafe { &*Current::<ObjSets>::new() };
    let tex_vertex = obj_sets[obj].1.objects[object].tex_vertices[tex_vertex];
    [tex_vertex.x, tex_vertex.y].into()
}}

dyon_fn!{fn normal_obj_object_normal
    (obj: usize, object: usize, normal: usize) -> Vec4 {
    let obj_sets = unsafe { &*Current::<ObjSets>::new() };
    let normal = obj_sets[obj].1.objects[object].normals[normal];
    [normal.x, normal.y, normal.z].into()
}}

dyon_fn!{fn vertices_obj_object(obj: usize, object: usize) -> Vec<Vec4> {
    let obj_sets = unsafe { &*Current::<ObjSets>::new() };
    obj_sets[obj].1.objects[object].vertices.iter()
        .map(|vertex| [vertex.x, vertex.y, vertex.z].into()).collect()
}}

dyon_fn!{fn tex_vertices_obj_object(obj: usize, object: usize) -> Vec<Vec4> {
    let obj_sets = unsafe { &*Current::<ObjSets>::new() };
    obj_sets[obj].1.objects[object].tex_vertices.iter()
        .map(|tex_vertex| [tex_vertex.x, tex_vertex.y].into()).collect()
}}

dyon_fn!{fn normals_obj_object(obj: usize, object: usize) -> Vec<Vec4> {
    let obj_sets = unsafe { &*Current::<ObjSets>::new() };
    obj_sets[obj].1.objects[object].normals.iter()
        .map(|normal| [normal.x, normal.y, normal.z].into()).collect()
}}

pub struct Geometry {
    pub material_name: Option<Arc<String>>,
    pub smooth_shading_group: usize,
    pub shapes: Vec<Shape>,
}

impl<'a> From<&'a obj::Geometry> for Geometry {
    fn from(val: &'a obj::Geometry) -> Geometry {
        Geometry {
            material_name: val.material_name.as_ref().map(|n| Arc::new(n.clone())),
            smooth_shading_group: val.smooth_shading_group,
            shapes: val.shapes.iter().map(|n| Shape(n.clone())).collect()
        }
    }
}

dyon_obj!{Geometry { material_name, smooth_shading_group, shapes }}

dyon_fn!{fn geometry_obj_object(obj: usize, object: usize) -> Vec<Geometry> {
    let obj_sets = unsafe { &*Current::<ObjSets>::new() };
    obj_sets[obj].1.objects[object].geometry.iter()
        .map(|geometry| geometry.into()).collect()
}}

/// Wraps a shape from OBJ library.
pub struct Shape(pub obj::Shape);

impl PopVariable for Shape {
    fn pop_var(rt: &Runtime, var: &Variable) -> Result<Self, String> {
        if let &Variable::Array(ref arr) = var {
            Ok(match arr.len() {
                1 => {
                    Shape(obj::Shape::Point(try!(rt.var(&arr[0]))))
                }
                2 => {
                    Shape(obj::Shape::Line(try!(rt.var(&arr[0])),
                                           try!(rt.var(&arr[1]))))
                }
                3 => {
                    Shape(obj::Shape::Triangle(try!(rt.var(&arr[0])),
                                               try!(rt.var(&arr[1])),
                                               try!(rt.var(&arr[2]))))
                }
                _ => return Err(rt.expected(var, "array of length 1, 2, 3"))
            })
        } else {
            Err(rt.expected(var, "array"))
        }
    }
}

impl PushVariable for Shape {
    fn push_var(&self) -> Variable {
        match self.0 {
            obj::Shape::Point(ref p) => Variable::Array(Arc::new(vec![p.push_var()])),
            obj::Shape::Line(ref a, ref b) => Variable::Array(Arc::new(vec![
                    a.push_var(), b.push_var()
                ])),
            obj::Shape::Triangle(ref a, ref b, ref c) => Variable::Array(Arc::new(vec![
                    a.push_var(), b.push_var(), c.push_var()
                ]))
        }
    }
}
