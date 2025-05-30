use clap::{Arg, Command};

pub fn swimlane_command() -> Command {
    Command::new("swimlane")
        .about("Create a new swimlane diagram")
        .arg(
            Arg::new("OUT")
                .short('o')
                .long("out")
                .value_name("FILE")
                .help("Output file for the diagram")
                .required(true)
                .action(clap::ArgAction::Set),
        )
        .arg(
            Arg::new("LANES")
                .short('l')
                .long("lanes")
                .value_name("LANES")
                .help("Number of swimlanes")
                .required(false)
                .value_parser(clap::value_parser!(usize))
                .action(clap::ArgAction::Set)
                .default_value("3"),
        )
}

pub fn process_swimlane_command(matches: &clap::ArgMatches) {
    static EMPTY_STRING: String = String::new();

    let lanes = matches
        .get_one::<usize>("LANES")
        .unwrap_or_else(|| &3)
        .clone();
    let file_out = matches
        .get_one::<String>("OUT")
        .unwrap_or_else(|| &EMPTY_STRING)
        .to_string();

    if let Err(e) = generate_swimlane_diagram(lanes, &file_out) {
        eprintln!("Error generating swimlane diagram: {}", e);
    } else {
        println!("Swimlane diagram generated successfully.");
    }
}

/// Generates an empty swimlane diagram with the specified number of lanes and writes it to the given file.
fn generate_swimlane_diagram(
    lanes: usize,
    file_out: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if lanes < 1 {
        return Err("Number of lanes must be at least 1".into());
    }
    if file_out.is_empty() {
        return Err("Output file path cannot be empty".into());
    }
    if !file_out.ends_with(".drawio") {
        return Err("Output file must have a `.drawio` extension".into());
    }
    println!(
        "Creating a swimlane diagram with {} lanes, outputting to {}",
        lanes, file_out
    );
    let mut body = Vec::with_capacity(13 + (lanes * 3));
    body.push(
    format!(
        r#"<mxfile host="Electron" version="27.0.9">
    <diagram name="Page-1" id="minimal-{lanes}-lanes">
    <mxGraphModel dx="1106" dy="883" grid="1" gridSize="10" guides="1" tooltips="1" connect="1" arrows="1" fold="1" page="1" pageScale="1" pageWidth="1100" pageHeight="850" background="none" math="0" shadow="0">
      <root>
        <mxCell id="0"/>
        <mxCell id="1" parent="0"/>
        <mxCell id="pool" value="Pool" style="swimlane;html=1;childLayout=stackLayout;startSize=20;rounded=0;shadow=0;labelBackgroundColor=none;strokeWidth=1;fontFamily=Verdana;fontSize=8;align=center;" vertex="1" parent="1">
          <mxGeometry x="70" y="40" width="960" height="750" as="geometry"/>
        </mxCell>
    "#
    ));
    let grow_by = 960 / lanes;
    let mut x = 0;
    for i in 1..=lanes {
        body.push(
            format!(
                r#"<mxCell id="lane{i}" value="Lane {i}" style="swimlane;html=1;startSize=20;" vertex="1" parent="pool">
          <mxGeometry x="{x}" y="20" width="{grow_by}" height="730" as="geometry"/>
        </mxCell>"#
            ));

        x += grow_by;
    }

    body.push(
        r#"</root>
    </mxGraphModel>
  </diagram>
</mxfile>"#
            .to_string(),
    );

    let body = body.join("\n");
    std::fs::write(file_out, body)?;

    Ok(())
}
