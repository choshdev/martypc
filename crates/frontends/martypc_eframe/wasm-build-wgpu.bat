set MARTYPC_URL_BASE=http://localhost:8080
set CARGO_UNSTABLE_BUILD_STD=std,panic_abort
trunk build --release=true --no-default-features --features=all_video_cards,sound,use_glow --config Trunk.toml
