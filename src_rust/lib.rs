use neon::prelude::*;

fn add_numbers(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let x = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let y = cx.argument::<JsNumber>(1)?.value(&mut cx);

    let result = x + y;

    Ok(cx.number(result))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("addNumbers", add_numbers)?;
    Ok(())
}
