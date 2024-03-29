#!/bin/bash
action=$1
if [[ -z "$action" || "$action" == "--help" ]]; then
  echo "====== poc rdev rust ======"
  echo "[g.] GIT:                         commit, status, push"
  echo "[pta.]create-tauri-app-command:   dev"
  echo "==============================================================="
else
 case $action in

  "g.commit")
        source cd-prr.sh
        git add .
        git commit -m "$1"
    ;;

    "g.push")
        source cd-prr.sh
        git add .
        git commit -m "$1"
        git push
    ;;

  "g.status")
        source cd-prr.sh
        git status
    ;;


## cd poc/create-tauri-app-command
    "pta.dev")
        source cd-prr.sh
        cd poc/create-tauri-app-command
        yarn tauri dev
    ;;


  *)
    # Handle unknown or missing arguments
    echo "Invalid or missing argument. Please specify a valid action or use --help for usage information."
    ;;

 esac
fi
