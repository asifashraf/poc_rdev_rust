#!/bin/bash
action=$1
if [[ -z "$action" || "$action" == "--help" ]]; then
  echo "  ====== Key listener ======"
  echo "  g.                  commit, status, push"


else
 case $action in
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
