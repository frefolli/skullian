#!/bin/bash
DEPTH=1

function arcan_analyze_target() {
    USER=$1
    REPO=$2
    ./arcan.sh analyze -i $USER/$REPO -l JAVA -o output --ignoreVCS metrics.componentMetrics='none' metrics.smellCharacteristics='none' metrics.projectMetrics='none' metrics.indexCalculators='none' output.writeDependencyGraph=true &> /dev/null
    mv output/arcanOutput/$REPO/dependency-graph-*.graphml arcan_$REPO.graphml
}

function skullian_analyze_target() {
    USER=$1
    REPO=$2
    cargo run -r -- -W -f java -l java $USER/$REPO -o skullian_$REPO.graphml &> /dev/null
}

function arcan_metrics() {
    USER=$1
    REPO=$2
    for (( i=1; i<=$DEPTH; i++ )); do
        arcan_preprocess
        START=`date +%s.%N`
        arcan_analyze_target $USER $REPO
        END=`date +%s.%N`
        runtime=$( echo "$END - $START" | bc -l )
        echo -e "$USER,$REPO,Arcan,$i,$runtime" >> benchmark.csv
        echo -e "$USER,$REPO,Arcan,$i,$runtime"
    done
}

function skullian_metrics() {
    USER=$1
    REPO=$2
    for (( i=1; i<=$DEPTH; i++ )); do
        START=`date +%s.%N`
        skullian_analyze_target $USER $REPO
        END=`date +%s.%N`
        runtime=$( echo "$END - $START" | bc -l )
        echo -e "$USER,$REPO,Skullian,$i,$runtime" >> benchmark.csv
        echo -e "$USER,$REPO,Skullian,$i,$runtime"
    done
}

function skullian_preprocess() {
    cargo build -r
}

function arcan_preprocess() {
    rm -rf output
}

function reset_metrics() {
    skullian_preprocess
    arcan_preprocess
}

function metrics_for() {
    USER=$1
    REPO=$2
    echo -e "skullian does: $USER/$REPO"
    skullian_analyze_target $USER $REPO
    echo -e "skullian done: $USER/$REPO"
    echo -e "arcan does: $USER/$REPO"
    arcan_analyze_target $USER $REPO
    echo -e "arcan done: $USER/$REPO"
}

function all_metrics() {
    reset_metrics
    metrics_for junit-team junit4
    metrics_for junit-team junit5
    metrics_for antlr antlr4
    metrics_for boh pruijt
    #metrics_for alibaba fastjson
    #metrics_for google guava
}

function jaccard_for() {
    USER=$1
    REPO=$2
    python -m main -Gj $REPO
}

function all_jaccard() {
    jaccard_for junit-team junit4
    jaccard_for junit-team junit5
    jaccard_for antlr antlr4
    jaccard_for boh pruijt
}

# all_metrics
all_jaccard
