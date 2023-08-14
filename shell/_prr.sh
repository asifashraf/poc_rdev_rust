#!/bin/bash
action=$1
if [[ -z "$action" || "$action" == "--help" ]]; then
  echo "  ====== poc rdev rust ======"
  echo "  g.                  commit, status, push"
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

  
  *)
    # Handle unknown or missing arguments
    echo "Invalid or missing argument. Please specify a valid action or use --help for usage information."
    ;;

 esac
fi
