use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;

pub struct ReadPriceDataPattern;

impl ReadPriceDataPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for ReadPriceDataPattern {
    fn name(&self) -> &'static str {
        "read_price_data"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }
        let body_text = block.body.join("\n");
        let looks_like = body_text.contains("Vec::<Val>::try_from_val")
            && body_text.contains("Vec::<Val>::from_val")
            && body_text.contains("Vec::<PriceData>::new")
            && body_text.contains("out_vec.push_back");
        if !looks_like {
            return None;
        }

        let mut feed_ids_var: Option<String> = None;
        let mut out_vec_var: Option<String> = None;
        let mut err_var: Option<String> = None;
        let mut has_err_var: Option<String> = None;
        let mut feed_fn_name: Option<String> = None;

        for l in &block.body {
            if feed_ids_var.is_none() {
                if let Some(pos) = l.find("Vec::<Val>::from_val(env, &val_from_i64(") {
                    let rest = &l[pos + "Vec::<Val>::from_val(env, &val_from_i64(".len()..];
                    if let Some(end) = rest.find("))") {
                        feed_ids_var = Some(rest[..end].trim().to_string());
                    }
                }
            }
            if out_vec_var.is_none() && l.contains("Vec::<PriceData>::new(env)") {
                if let Some(pos) = l.find("let mut ") {
                    let rest = &l[pos + "let mut ".len()..];
                    if let Some(end) = rest.find(":") {
                        out_vec_var = Some(rest[..end].trim().to_string());
                    }
                }
            }
            if err_var.is_none() && l.contains("let mut ") && l.contains(": i64") && l.contains("err") {
                if let Some(pos) = l.find("let mut ") {
                    let rest = &l[pos + "let mut ".len()..];
                    if let Some(end) = rest.find(":") {
                        err_var = Some(rest[..end].trim().to_string());
                    }
                }
            }
            if has_err_var.is_none() && l.contains("let mut ") && l.contains(": i32") && l.contains("has_err") {
                if let Some(pos) = l.find("let mut ") {
                    let rest = &l[pos + "let mut ".len()..];
                    if let Some(end) = rest.find(":") {
                        has_err_var = Some(rest[..end].trim().to_string());
                    }
                }
            }
            if feed_fn_name.is_none() && l.contains("read_price_data_for_feed") {
                feed_fn_name = Some("read_price_data_for_feed".to_string());
            }
        }

        let feed_ids_var = feed_ids_var.unwrap_or_else(|| "feed_ids".to_string());
        let out_vec_var = out_vec_var.unwrap_or_else(|| "out_vec".to_string());
        let err_var = err_var.unwrap_or_else(|| "err".to_string());
        let has_err_var = has_err_var.unwrap_or_else(|| "has_err".to_string());
        let feed_fn_name = feed_fn_name.unwrap_or_else(|| "read_price_data_for_feed".to_string());

        let inner = format!("{}    ", block.indent);
        let mut new_body: Vec<String> = Vec::new();
        new_body.push(format!(
            "{}if (Vec::<Val>::try_from_val(env, &val_from_i64({})).is_ok()) as i32 != 0 {{",
            inner, feed_ids_var
        ));
        new_body.push(format!(
            "{}    let feed_vec = Vec::<Val>::from_val(env, &val_from_i64({}));",
            inner, feed_ids_var
        ));
        new_body.push(format!(
            "{}    let mut {} = Vec::<PriceData>::new(env);",
            inner, out_vec_var
        ));
        new_body.push(format!("{}    let mut {}: i64 = 0;", inner, err_var));
        new_body.push(format!("{}    let mut {}: i32 = 0;", inner, has_err_var));
        new_body.push(format!(
            "{}    self.for_each_val(env, &feed_vec, |val| {{",
            inner
        ));
        new_body.push(format!(
            "{}        if {} != 0 {{ return; }}",
            inner, has_err_var
        ));
        new_body.push(format!("{}        let feed = String::from_val(env, &val);", inner));
        new_body.push(format!(
            "{}        match self.{}(env.clone(), feed) {{",
            inner, feed_fn_name
        ));
        new_body.push(format!(
            "{}            Ok(v) => {}.push_back(v),",
            inner, out_vec_var
        ));
        new_body.push(format!(
            "{}            Err(e) => {{ {} = 1; {} = val_to_i64(e.into_val(env)); }}",
            inner, has_err_var, err_var
        ));
        new_body.push(format!("{}        }}", inner));
        new_body.push(format!("{}    }});", inner));
        new_body.push(format!(
            "{}    if {} != 0 {{ return {}; }}",
            inner, has_err_var, err_var
        ));
        new_body.push(format!(
            "{}    return val_to_i64({}.into_val(env));",
            inner, out_vec_var
        ));
        new_body.push(format!("{}}}", inner));
        new_body.push(format!("{}unreachable!();", inner));
        new_body.push(format!("{}unreachable!();", inner));

        Some(FunctionBlock {
            header: block.header.clone(),
            body: new_body,
            footer: block.footer.clone(),
            indent: block.indent.clone(),
            name: block.name.clone(),
        })
    }
}
