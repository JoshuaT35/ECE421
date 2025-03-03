use yew::prelude::*;
extern crate meval;

#[function_component(App)]
fn app() -> Html {
    // variable in calculator input field
    let result = use_state(|| "".to_string());

    // resolve using equal sign
    let equal_solve = {
        let result = result.clone();
        Callback::from(move |_| {
            let expr = (*result).clone();
            if let Ok(value) = meval::eval_str(&expr) {
                result.set(value.to_string());
            } else {
                result.set("Error".to_string());
            }
        })
    };

    // Resolve using the natural logarithm (ln)
    let ln_solve = {
        let result = result.clone();
        Callback::from(move |_| {
            let expr = (*result).clone();
            
            // parse the result as a floating-point number
            if let Ok(value) = expr.parse::<f64>() {
                if value > 0.0 {
                    // calculate ln(value)
                    let ln_value = value.ln();
                    result.set(ln_value.to_string());
                }
                else {
                    result.set("ln error: value must be positive".to_string());
                }
            }
            else {
                result.set("ln error: Invalid input".to_string());
            }
        })
    };

    // updates result variable through a passed input
    let update_field = {
        let result = result.clone();
        Callback::from(move |value: String| {
            let value = format!("{}{}", *result, value);
            result.set(value);
        })
    };

    // Recognizes key press events and updates the result variable.
    let onkeydown = Callback::from({
        let result = result.clone();
        move |event:KeyboardEvent|{
            if event.key() == "Enter" {
                let expr = (*result).clone();
                if let Ok(value) = meval::eval_str(&expr) {
                    result.set(value.to_string());
                } else {
                    result.set("Error".to_string());
                }
            }
            else if event.key() == "l" {
                let expr = (*result).clone();
            
                // parse the result as a floating-point number
                if let Ok(value) = expr.parse::<f64>() {
                    if value > 0.0 {
                        // calculate ln(value)
                        let ln_value = value.ln();
                        result.set(ln_value.to_string());
                    }
                    else {
                        result.set("ln error: value must be positive".to_string());
                    }
                }
            }
            else if event.key() == "1" 
                || event.key() == "2" 
                || event.key() == "3"
                || event.key() == "4"
                || event.key() == "5"
                || event.key() == "6"
                || event.key() == "7"
                || event.key() == "8"
                || event.key() == "9"
                || event.key() == "0"
                || event.key() == "^"
                || event.key() == "%"
                || event.key() == "l"
                || event.key() == "."
                || event.key() == "*"
                || event.key() == "+"
                || event.key() == "/"
                || event.key() == "-" {
                // only append to result if one of these keys is pressed
                let value = format!("{}{}", *result, event.key());
                result.set(value);
            }
        }
    });
    
    // clear the result field
    let clear_field = {
        let result = result.clone();
        Callback::from(move |_| {
            result.set("".to_string());
        })
    };

    html! {
        <div>
            // picture
            <img src="/static/calc.jpg" alt="calcpic" width="300" height="200"/>
            // header
            // <!-- Use Table to Create Calculator Structure Design -->
            <table id="calcu">
                <tr>
                    // input field, clear
                    <td colspan=3><input type="text" value={(*result).clone()} onkeydown={onkeydown} readonly=true/> </td>
                    <td><input type="button" value="c" onclick={clear_field}/> </td>
                </tr>
                <tr>
                    // 1, 2, 3, /
                    <td><input type="button" value="1" onclick={update_field.reform(|_| "1".to_string())}/> </td>
                    <td><input type="button" value="2" onclick={update_field.reform(|_| "2".to_string())}/> </td>
                    <td><input type="button" value="3" onclick={update_field.reform(|_| "3".to_string())}/> </td>
                    <td><input type="button" value="/" onclick={update_field.reform(|_| "/".to_string())}/> </td>
                </tr>
                <tr>
                    // 4, 5, 6, *
                    <td><input type="button" value="4" onclick={update_field.reform(|_| "4".to_string())}/> </td>
                    <td><input type="button" value="5" onclick={update_field.reform(|_| "5".to_string())}/> </td>
                    <td><input type="button" value="6" onclick={update_field.reform(|_| "6".to_string())}/> </td>
                    <td><input type="button" value="*" onclick={update_field.reform(|_| "*".to_string())}/> </td>
                </tr>
                <tr>
                    // 7, 8, 9, -
                    <td><input type="button" value="7" onclick={update_field.reform(|_| "7".to_string())}/> </td>
                    <td><input type="button" value="8" onclick={update_field.reform(|_| "8".to_string())}/> </td>
                    <td><input type="button" value="9" onclick={update_field.reform(|_| "9".to_string())}/> </td>
                    <td><input type="button" value="-" onclick={update_field.reform(|_| "-".to_string())}/> </td>
                </tr>
                <tr>
                    // 0, ., =, +
                    <td><input type="button" value="0" onclick={update_field.reform(|_| "0".to_string())}/> </td>
                    <td><input type="button" value="." onclick={update_field.reform(|_| ".".to_string())}/> </td>
                    <td><input type="button" value="=" onclick={equal_solve}/> </td>
                    <td><input type="button" value="+" onclick={update_field.reform(|_| "+".to_string())}/> </td>
                </tr>
                <tr>
                    // ^, %, ln
                    <td><input type="button" value="^" onclick={update_field.reform(|_| "^".to_string())}/> </td>
                    <td><input type="button" value="%" onclick={update_field.reform(|_| "%".to_string())}/> </td>
                    <td><input type="button" value="ln" onclick={ln_solve}/> </td>
                </tr>
            </table>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}