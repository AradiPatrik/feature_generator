use convert_case::{Case, Casing};
use handlebars::{Context, Handlebars, Helper, HelperResult, JsonRender, Output, RenderContext};

pub fn to_flat(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param = h.param(0).unwrap();

    out.write(
        param
            .value()
            .render()
            .to_string()
            .to_case(Case::Flat)
            .as_ref(),
    )?;
    Ok(())
}

pub fn to_pascal(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param = h.param(0).unwrap();

    out.write(
        param
            .value()
            .render()
            .to_string()
            .to_case(Case::Pascal)
            .as_ref(),
    )?;
    Ok(())
}

pub fn to_camel(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param = h.param(0).unwrap();

    out.write(
        param
            .value()
            .render()
            .to_string()
            .to_case(Case::Camel)
            .as_ref(),
    )?;
    Ok(())
}

pub fn to_kebab(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param = h.param(0).unwrap();

    out.write(
        param
            .value()
            .render()
            .to_string()
            .to_case(Case::Kebab)
            .as_ref(),
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn todo() {}
}
