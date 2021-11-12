docker run -it -d \
    --gpus '"device=0"' --ipc=host \
    --net some_net \
    -e SOME_VAR='some_value' \
    -v $HOME/data:/data \
    some_image \
    python run.py --n_trials 5 --bs 128 --img_sz 448 --arch resnet18
