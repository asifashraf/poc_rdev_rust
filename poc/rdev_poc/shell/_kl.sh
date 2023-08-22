#!/bin/bash
action=$1
if [[ -z "$action" || "$action" == "--help" ]]; then
  echo "  ====== Key listener ======"
  echo "                      run, build, clean, build-release, update-dep, test, doc, format, publish-pkg"
  #echo "  r.  (run)           main, keys, keys-db, keys-rl"
  echo "  g.                  commit, status, push"
else
 case $action in

 "run")
         source cd-kl.sh
         cargo run
     ;;


 "clean")
         source cd-kl.sh
         cargo clean
     ;;

  "r.main")
        source cd-kl.sh
        cargo run --bin main
    ;;

  "r.keys")
        source cd-kl.sh
        cargo run --bin keys
    ;;

  "r.keys-db")
        source cd-kl.sh
        cd target/debug
        ./keys
    ;;

  "r.keys-rl")
        source cd-kl.sh
        cd target/release
        ./keys
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
