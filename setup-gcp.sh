#!/bin/bash
set -e

# Function to display commands
exe() { printf "\$ \033[35m"; echo "$@"; printf "\033[90m";  $@; printf "\033[0m\n"; }

project=
project_name=
bucket=
location="europe-west4"

while getopts 'p:n:b:l:h' opt; do
  case "$opt" in
    p)
      project="$OPTARG"
      ;;

    n)
      project_name="$OPTARG"
      ;;

    b)
      bucket="$OPTARG"
      ;;

    l)
      location="$OPTARG"
      ;;
   
    ?|h)
      echo "Usage: $(basename $0) [-p project] [-n project_name] [-b bucket] [-l location]"
      exit 1
      ;;
  esac
done

if [ -z "$project" ]; then
    # TODO: what if there is no default project?
    project=$(gcloud config get-value project)
fi

if [ -z "$project_name" ]; then
    # TODO: what if the project does not exist?
    project_name=$(gcloud projects describe $project --format="value(name)")
fi

if [ -z "$bucket" ]; then
    bucket="bucket"
fi
bucket="${project_name}-$bucket"

echo "project identifier: $project"
echo "project name      : $project_name"
echo "bucket identifier : $bucket"
echo "storage location  : $location"
echo ""

echo "create project($project) ..."
if gcloud projects list --format="value(project_id)" | grep -w -E "^$project\$" > /dev/null ; then
    printf "[\033[33mskip\033[0m] already exists\n"
else
    # TODO: find reason for quota exceeded error when creating project
    exe gcloud projects create $project --name=$project_name
    printf "[\033[92mok\033[0m] project created\n"
fi
echo ""

echo "enabling 'storage.googleapis.com' service ..."
if gcloud services list --project $project --enabled | grep -w -E "^storage.googleapis.com" > /dev/null ; then
    printf "[\033[33mskip\033[0m] already enabled\n"
else
    exe gcloud services enable \
        storage.googleapis.com \
        --project=$project
    printf "[\033[92mok\033[0m] service enabled\n"
fi
echo ""

echo "create storage bucket($bucket) ..."
if gcloud storage buckets list --format="value(name)" | grep -w "$bucket" > /dev/null ; then
    printf "[\033[33mskip\033[0m] already exists\n"
else
    exe gcloud storage buckets create \
        gs://$bucket \
        --project=$project \
        --public-access-prevention \
        --uniform-bucket-level-access \
        --default-storage-class=standard \
        --location=$location
    printf "[\033[92mok\033[0m] storage bucket created\n"
fi
echo ""

echo "setup completed"
