// Knockway Inc. and its affiliates. All Rights Reserved

extern crate bindgen;
extern crate cc;

use std::process::Command;
use std::env;
use std::path::Path;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.lock");
    Command::new("./bootstrap.sh")
                      .current_dir(&Path::new("libpostal"))
                      .status().unwrap();
    Command::new("./configure")
                      .current_dir(&Path::new("libpostal"))
                      .status().unwrap();
    let sources = vec!(
        "strndup.c",
        "libpostal.c",
        "expand.c",
        "address_dictionary.c",
        "transliterate.c",
        "tokens.c",
        "trie.c",
        "trie_search.c",
        "trie_utils.c",
        "string_utils.c",
        "file_utils.c",
        "utf8proc/utf8proc.c",
        "normalize.c",
        "numex.c",
        "features.c",
        "unicode_scripts.c",
        "address_parser.c",
        "address_parser_io.c",
        "averaged_perceptron.c",
        "crf.c",
        "crf_context.c",
        "sparse_matrix.c",
        "averaged_perceptron_tagger.c",
        "graph.c",
        "graph_builder.c",
        "language_classifier.c",
        "language_features.c",
        "logistic_regression.c",
        "logistic.c",
        "minibatch.c",
        "float_utils.c",
        "ngrams.c",
        "place.c",
        "near_dupe.c",
        "double_metaphone.c",
        "geohash/geohash.c",
        "dedupe.c",
        "string_similarity.c",
        "acronyms.c",
        "soft_tfidf.c",
        "jaccard.c",
        "klib/drand48.c",
        "scanner.c",
    );
    let mut build = cc::Build::new();
    for source in sources {
        build.file(format!("libpostal/src/{}", source));
    }
    build.define("LIBPOSTAL_DATA_DIR", "\"/tmp/libpostal\"")
    .define("DOWNLOAD_DATA","false")
    .define("HAVE_CONFIG_H","1")
    .warnings(false)
    .extra_warnings(false)
    .include("libpostal")
    .compile("libpostal.a");

    let bindings = bindgen::Builder::default()
        .rustfmt_bindings(true)
        .header("wrapper.h")
        .derive_debug(true)
        .trust_clang_mangling(false)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
