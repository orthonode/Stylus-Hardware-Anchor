idf_component_register(
SRCS "anchor_ohr.cpp" "tiny-keccak.c"
INCLUDE_DIRS "."
PRIV_REQUIRES
nvs_flash
app_update
efuse
esp_app_format
)
