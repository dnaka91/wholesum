use std::{
    fs::File,
    io::{self, Result as IoResult, Write},
    path::{Path, PathBuf},
};

use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::Shell;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use wholesum::{hashfile::HashFile, Algorithm, Mode};

#[derive(Parser)]
#[command(about, author, version)]
struct Opt {
    /// Text or binary mode.
    ///
    /// Doesn't affect hashing output at all and is just there for compatibility with other hashing
    /// tools. It will affect whether the file path in the output is prefixed with a space ' '
    /// (text) or asterisk '*' (binary).
    #[arg(short, long, value_enum, default_value_t)]
    mode: Mode,
    /// The hashing algorithm that shall be used.
    ///
    /// The list of available algorithms is grouped into common and uncommon and then sorted by
    /// strength. Early entries in the list are considered common and extremely strong, later
    /// entries rather rare and weak algorithms.
    #[arg(short, long, value_enum, default_value_t)]
    algorithm: Algorithm,
    /// Prepend each output line with the used hash algorithm.
    ///
    /// This changes the default output format used by many other hashing tools.
    ///
    /// It allows to later easily combine several files with different algorithms together or even
    /// check the same files multiple times with different algorithms.
    #[arg(short, long)]
    prefix: bool,
    /// List of files to hash. Directories are silently ignored.
    files: Vec<PathBuf>,
    #[command(subcommand)]
    cmd: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Check hashes against their actual files.
    Check {
        /// File containing the list of hashes.
        file: PathBuf,
    },
    /// Generate shell completions, writing them to the standard output.
    Completions {
        /// The shell type to generate completions for.
        #[arg(value_enum)]
        shell: Shell,
    },
    /// Generate man pages, writing them to the given directory.
    Manpages {
        /// Target directory.
        dir: PathBuf,
    },
}

/// Raw OS error code happening when trying to read a directory as file.
const IS_A_DIRECTORY: i32 = 21;

fn main() -> Result<()> {
    let opt = Opt::parse();

    if let Some(cmd) = opt.cmd {
        match cmd {
            Command::Check { file } => verify_files(file),
            Command::Completions { shell } => {
                print_completions(shell);
                Ok(())
            }
            Command::Manpages { dir } => print_manpages(&dir),
        }
    } else {
        hash_files(opt)
    }
}

fn verify_files(file: PathBuf) -> Result<()> {
    let hash_file = HashFile::parse(file)?;

    let matches = hash_file
        .entries
        .into_par_iter()
        .map(|entry| {
            let hasher = entry
                .algorithm
                .or(hash_file.algorithm)
                .unwrap_or(Algorithm::Blake3)
                .into_hasher();

            let hash = hasher(Path::new(&entry.file))?;

            Ok((entry.file, hash == entry.hash))
        })
        .collect::<Result<Vec<_>>>()?;

    let width = matches
        .iter()
        .map(|(file, _)| file.len())
        .max()
        .unwrap_or_default();

    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();

    for (file, valid) in matches {
        writeln!(
            stdout,
            "{:width$} {}",
            file,
            if valid { "OK" } else { "ERR" },
            width = width
        )?;
    }

    stdout.flush()?;

    Ok(())
}

fn print_completions(shell: Shell) {
    clap_complete::generate(
        shell,
        &mut Opt::command(),
        env!("CARGO_PKG_NAME"),
        &mut io::stdout().lock(),
    );
}

fn print_manpages(dir: &Path) -> Result<()> {
    fn print(dir: &Path, app: &clap::Command) -> Result<()> {
        let name = app.get_display_name().unwrap_or_else(|| app.get_name());
        let mut out = File::create(dir.join(format!("{name}.1")))?;

        clap_mangen::Man::new(app.clone()).render(&mut out)?;
        out.flush()?;

        for sub in app.get_subcommands() {
            print(dir, sub)?;
        }

        Ok(())
    }

    let mut app = Opt::command();
    app.build();

    print(dir, &app)
}

fn hash_files(opt: Opt) -> Result<()> {
    let hasher = opt.algorithm.into_hasher();

    let mut hashes = opt
        .files
        .into_par_iter()
        .filter_map(|file| match hasher(&file) {
            Ok(hash) => Some(Ok((hash, file))),
            Err(e) if e.raw_os_error() == Some(IS_A_DIRECTORY) => None,
            Err(e) => Some(Err(e)),
        })
        .collect::<IoResult<Vec<_>>>()?;

    hashes.sort_unstable_by(|(_, a), (_, b)| a.cmp(b));

    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();

    for (hash, path) in hashes {
        if opt.prefix {
            write!(stdout, "{} ", opt.algorithm)?;
        }

        writeln!(stdout, "{} {}{}", hash, opt.mode, path.display())?;
    }

    stdout.flush()?;

    Ok(())
}
