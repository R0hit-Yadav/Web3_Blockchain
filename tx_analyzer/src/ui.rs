use crate::blockchain::Transaction;
use crossterm::{
    event::{self, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use petgraph::dot::{Dot, Config};
use petgraph::graph::DiGraph;
use std::io;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, Wrap},
    Terminal,
};

pub fn run_ui(user_address: &str, transactions: &[Transaction], graph: &DiGraph<String, String>) -> Result<(), Box<dyn std::error::Error>> {
    println!("length is {}",transactions.len());
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // println!("user address is {}",user_address);
    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(f.size());

            let header = Row::new(vec!["From", "To", "Value (ETH)", "Txn Hash"])
                .style(Style::default().add_modifier(Modifier::BOLD));
            let rows: Vec<Row> = transactions.iter().map(|tx| {
                Row::new(vec![
                    Cell::from(tx.from.to_string()),
                    Cell::from(tx.to.to_string()),
                    Cell::from(format!("{}", ethers::utils::format_units(tx.value, "ether").unwrap_or_default())),
                    Cell::from(safe_truncate(&tx.hash, 10)), // Prevent slicing issues
                ])
            }).collect();
            let table = Table::new(rows)
                .header(header)
                .block(Block::default().title(" Transactions ").borders(Borders::ALL))
                .widths(&[
                    Constraint::Length(30),
                    Constraint::Length(30),
                    Constraint::Length(30),
                    Constraint::Length(30), // Adjusted width
                ]);

            let graph_string = graph
                .edge_indices()
                .map(|edge| {
                    let (from, to) = graph.edge_endpoints(edge).unwrap();
                    let edge_label = graph[edge].clone();
                    let from_node = graph[from].as_str();
                    let to_node = graph[to].as_str();

                    if from_node == user_address {
                        format!(
                            "{} ─({})-> {}",
                            safe_truncate(from_node, 10),
                            safe_truncate(&edge_label, 10),
                            safe_truncate(to_node, 10)
                        )
                    }
                    else if to_node == user_address {
                        format!(
                            "{} <─({})─ {}",
                            safe_truncate(from_node, 10),
                            safe_truncate(&edge_label, 10),
                            safe_truncate(to_node, 10)
                        )
                    } 
                    else {
                        format!(
                            "{} ─({})─> {}",
                            safe_truncate(from_node, 10),
                            safe_truncate(&edge_label, 10),
                            safe_truncate(to_node, 10)
                        )
                    } 
                })
                .collect::<Vec<String>>()
                .join("\n");

            let graph_paragraph = Paragraph::new(format!("Transaction Flow:\n{}", graph_string))
                .block(Block::default().title(" Graph ").borders(Borders::ALL))
                .wrap(Wrap { trim: false });

            f.render_widget(table, chunks[0]);
            f.render_widget(graph_paragraph, chunks[1]);
            
        })?;

        if let event::Event::Key(key) = event::read()? 
        {
            if key.code == KeyCode::Esc {
                break;
            }
        }
    }

    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}

fn safe_truncate(s: &str, max_len: usize) -> String {
    s.chars().take(max_len).collect()
}
