exports.nearest_base = (v, base) => {
    return base * Math.ceil(v / base)
}

exports.getStream = async () => {
    return navigator.mediaDevices.getUserMedia({
        audio: false,
        video: {
            width: 1280,
            height: 720
        }
    })
}
