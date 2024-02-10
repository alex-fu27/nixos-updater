## Updater states
1. Evaluate (flake with update-inputs, would require channel update before that)
    Possible results: ouput differs to current system (upgrade available) or not or evaluation failed
2. Build
    Possible results: build failed, upgrade ready without kernel, upgrade
ready with kernel
3. Switching
    Possible results: done, error
