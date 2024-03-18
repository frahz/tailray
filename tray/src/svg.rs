use usvg::Tree;

const MARK_WHITE_24: &'static str = r##"
<svg width="26" height="26" viewBox="0 0 26 26" fill="none" xmlns="http://www.w3.org/2000/svg"><g clip-path="url(#clip0_13627_11860)"><path opacity="0.2" d="M3.8696 6.77137C5.56662 6.77137 6.94233 5.39567 6.94233 3.69865C6.94233 2.00163 5.56662 0.625919 3.8696 0.625919C2.17258 0.625919 0.796875 2.00163 0.796875 3.69865C0.796875 5.39567 2.17258 6.77137 3.8696 6.77137Z" fill="black" style="--darkreader-inline-fill: #f8eddf;" data-darkreader-inline-fill=""></path><path d="M3.8696 15.9327C5.56662 15.9327 6.94233 14.5569 6.94233 12.8599C6.94233 11.1629 5.56662 9.7872 3.8696 9.7872C2.17258 9.7872 0.796875 11.1629 0.796875 12.8599C0.796875 14.5569 2.17258 15.9327 3.8696 15.9327Z" fill="black" style="--darkreader-inline-fill: #f8eddf;" data-darkreader-inline-fill=""></path><path opacity="0.2" d="M3.8696 25.2646C5.56662 25.2646 6.94233 23.8889 6.94233 22.1919C6.94233 20.4949 5.56662 19.1192 3.8696 19.1192C2.17258 19.1192 0.796875 20.4949 0.796875 22.1919C0.796875 23.8889 2.17258 25.2646 3.8696 25.2646Z" fill="black" style="--darkreader-inline-fill: #f8eddf;" data-darkreader-inline-fill=""></path><path d="M13.0879 15.9327C14.7849 15.9327 16.1606 14.5569 16.1606 12.8599C16.1606 11.1629 14.7849 9.7872 13.0879 9.7872C11.3908 9.7872 10.0151 11.1629 10.0151 12.8599C10.0151 14.5569 11.3908 15.9327 13.0879 15.9327Z" fill="black" style="--darkreader-inline-fill: #f8eddf;" data-darkreader-inline-fill=""></path><path d="M13.0879 25.2646C14.7849 25.2646 16.1606 23.8889 16.1606 22.1919C16.1606 20.4949 14.7849 19.1192 13.0879 19.1192C11.3908 19.1192 10.0151 20.4949 10.0151 22.1919C10.0151 23.8889 11.3908 25.2646 13.0879 25.2646Z" fill="black" style="--darkreader-inline-fill: #f8eddf;" data-darkreader-inline-fill=""></path><path opacity="0.2" d="M13.0879 6.77137C14.7849 6.77137 16.1606 5.39567 16.1606 3.69865C16.1606 2.00163 14.7849 0.625919 13.0879 0.625919C11.3908 0.625919 10.0151 2.00163 10.0151 3.69865C10.0151 5.39567 11.3908 6.77137 13.0879 6.77137Z" fill="black" style="--darkreader-inline-fill: #f8eddf;" data-darkreader-inline-fill=""></path><path opacity="0.2" d="M22.1919 6.77137C23.8889 6.77137 25.2646 5.39567 25.2646 3.69865C25.2646 2.00163 23.8889 0.625919 22.1919 0.625919C20.4948 0.625919 19.1191 2.00163 19.1191 3.69865C19.1191 5.39567 20.4948 6.77137 22.1919 6.77137Z" fill="black" style="--darkreader-inline-fill: #f8eddf;" data-darkreader-inline-fill=""></path><path d="M22.1919 15.9327C23.8889 15.9327 25.2646 14.5569 25.2646 12.8599C25.2646 11.1629 23.8889 9.7872 22.1919 9.7872C20.4948 9.7872 19.1191 11.1629 19.1191 12.8599C19.1191 14.5569 20.4948 15.9327 22.1919 15.9327Z" fill="black" style="--darkreader-inline-fill: #f8eddf;" data-darkreader-inline-fill=""></path><path opacity="0.2" d="M22.1919 25.2646C23.8889 25.2646 25.2646 23.8889 25.2646 22.1919C25.2646 20.4949 23.8889 19.1192 22.1919 19.1192C20.4948 19.1192 19.1191 20.4949 19.1191 22.1919C19.1191 23.8889 20.4948 25.2646 22.1919 25.2646Z" fill="black" style="--darkreader-inline-fill: #f8eddf;" data-darkreader-inline-fill=""></path></g><defs><clipPath id="clip0_13627_11860"><rect width="26" height="26" fill="white" style="--darkreader-inline-fill: #f8eddf;" data-darkreader-inline-fill=""></rect></clipPath></defs></svg>
"##;

pub fn load_icon(enabled: bool) -> Vec<ksni::Icon> {
    match enabled {
        true => vec![to_icon(MARK_WHITE_24)],
        false => vec![to_icon(&MARK_WHITE_24.replace("1.0", "0.4"))],
    }
}
fn to_icon(svg_str: &str) -> ksni::Icon {
    let rtree = Tree::from_str(svg_str, &usvg::Options::default().to_ref()).unwrap();
    let pixmap_size = rtree.svg_node().size;
    let mut pixmap = tiny_skia::Pixmap::new(
        pixmap_size.width().round() as u32,
        pixmap_size.height().round() as u32,
    )
    .unwrap();

    resvg::render(
        &rtree,
        usvg::FitTo::Original,
        tiny_skia::Transform::default(),
        pixmap.as_mut(),
    )
    .unwrap();

    ksni::Icon {
        width: pixmap.width() as i32,
        height: pixmap.height() as i32,
        data: pixmap.take(),
    }
}
