use syn::{GenericArgument, Path, PathArguments, PathSegment, Type, TypePath};

pub fn is_option_type(ty: &Type) -> bool {
    match last_path_segment(ty) {
        Some(path_seg) => path_seg.ident == "Option",
        None => false,
    }
}

pub fn is_vec_type(ty: &Type) -> bool {
    match last_path_segment(ty) {
        Some(path_seg) => path_seg.ident == "Vec",
        None => false,
    }
}

pub fn extract_inner_type(ty: &Type) -> &GenericArgument {
    match last_path_segment(ty) {
        Some(PathSegment {
            ident: _,
            arguments: PathArguments::AngleBracketed(ref gen_arg),
        }) => gen_arg.args.first(),
        _ => None,
    }
    .expect("invalid option type")
}

pub fn last_path_segment(ty: &Type) -> Option<&PathSegment> {
    match ty {
        &Type::Path(TypePath {
            qself: None,
            path:
                Path {
                    segments: ref seg,
                    leading_colon: _,
                },
        }) => seg.last(),
        _ => None,
    }
}
