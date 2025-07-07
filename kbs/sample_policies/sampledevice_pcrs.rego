package policy
import rego.v1

default allow = false

# Reference PCR hashes
pcr10_hash = "9dbb22387e019c04da95477333546082ce313152c4a7f8a6e1d5f0c77f4fb6f0"
pcr1_hash = "0000000000000000000000000000000000000000000000000000000000000000"

#allow if {
#    input["submods"]["cpu0"]["ear.status"] == "contraindicated" 
#}

#allow if {
#    some cpu_key
#    submod := input.submods[cpu_key]
#    additional := submod["ear.veraison.annotated-evidence"].runtime_data_claims["additional-evidence"]
#    contains(additional, target_hash)
#}

#allow if {
  # Check all submodules
#  some mod
#  submod := input.submods[mod]
#  additional_raw := submod["ear.veraison.annotated-evidence"].runtime_data_claims["additional-evidence"]
#  additional_obj := json.unmarshal(additional_raw)

  # Check all devices
#  some device
#  device_obj := additional_obj[device]
#  pcr := device_obj["tpm_quote"]["pcrs"][_]
#  pcr == target_hash
#}

# This rule unmarshalls the additional evidence and checks for specific PCR values
# This is a more specific rule that checks for the exact PCR values we are interested in
allow if {
  input["submods"]["cpu0"]["ear.status"] == "contraindicated" 
  submod := input.submods["cpu0"]
  additional_raw := submod["ear.veraison.annotated-evidence"].runtime_data_claims["additional-evidence"]
  additional_obj := json.unmarshal(additional_raw)

  # Access device (e.g. "sampledevice")  
  device_obj := additional_obj["sampledevice"]

  # Access PCR indexes and compare
  device_obj["tpm_quote"]["pcrs"][1] == pcr1_hash
  device_obj["tpm_quote"]["pcrs"][10] == pcr10_hash
}