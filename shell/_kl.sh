#!/bin/bash
action=$1
if [[ -z "$action" || "$action" == "--help" ]]; then
  echo "  ====== Key listener ======"
  echo "                      build, build-release, update-dep, test, doc, format, publish-pkg"
  echo "  r.  (run)           main, keys"
  echo "  g.                  commit, status, push"
else
 case $action in

  "r.main")
        source cd-kl.sh
        cargo run --bin main
    ;;

  "r.keys")
        source cd-kl.sh
        cargo run --bin keys
    ;;

  "build")
        source cd-kl.sh
        cargo build
    ;;

  "build-release")
        source cd-kl.sh
        cargo build --release
    ;;

  "update-dep")
    source cd-kl.sh
    cargo update
  ;;

  "test")
    source cd-kl.sh
    cargo test
  ;;

  "check")
    source cd-kl.sh
    cargo check
  ;;

  "doc")
    source cd-kl.sh
    cargo doc --open
  ;;

  "format")
    source cd-kl.sh
    cargo fmt
  ;;


  "public-pkg")
    source cd-kl.sh
    cargo publish
  ;;

  "g.commit")
        source cd-kl.sh
        git add .
        git commit -m "$1"
    ;;
    "g.push")
        source cd-kl.sh
        git add .
        git commit -m "$1"
        git push
    ;;
  "g.status")
        source cd-kl.sh
        git status
    ;;

  
  *)
    # Handle unknown or missing arguments
    echo "Invalid or missing argument. Please specify a valid action or use --help for usage information."
    ;;

 esac
fi
