/*const path = require("path")
const fs = require("fs")
const { execSync } = require("child_process")


function get_meta() {
    let cmd = path.resolve(__dirname, "../mona/target/release/gen_tf_meta.exe")
    if (!fs.existsSync(cmd)) {
        cmd = path.resolve(__dirname, "../mona/target/debug/gen_tf_meta.exe")
    }
    if (!fs.existsSync(cmd)) {
        cmd = "cargo run --bin gen_tf_meta"
    }
    console.log(cmd)

    let stdout = execSync(cmd)
    let ret = stdout.toString();
    // console.log(ret)

    return ret
}

// get_weapon_meta()

module.exports = function (source) {
    return get_meta()
}
*/