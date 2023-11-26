NAME="${1##*/}"
BIN="${1}.bin"
arm-none-eabi-size "${1}"
arm-none-eabi-objcopy -O binary "${1}" "${BIN}"

cat > project.pros << EOL
{
    "py/object": "pros.conductor.project.Project",
    "py/state": {
        "project_name": "${NAME} competition v3",
        "target": "v5",
        "templates": {
            "kernel": {
                "location": "",
                "metadata": {
                    "origin": "pros-mainline",
                    "output": "${BIN}"
                },
                "name": "kernel",
                "py/object": "pros.conductor.templates.local_template.LocalTemplate",
                "supported_kernels": null,
                "system_files": [],
                "target": "v5",
                "user_files": [],
                "version": "3.3.1"
            }
        },
        "upload_options": {
            "icon": "clawbot",
            "slot": 2,
            "description": "v3 of robot competition code"
        }
    }
}
EOL

prosv5 ut
