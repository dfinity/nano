load ../utils/_

setup() {
    standard_nns_setup
}

teardown() {
    standard_nns_teardown
}

@test "basic create neuron" {
    #account is initialized with 10_000 tokens
    assert_command quill account-balance 345f723e9e619934daac6ae0f4be13a7b0ba57d6a608e511a00fd0ded5866752 --yes
    assert_string_match '(record { e8s = 1_000_000_000_000 : nat64 })'

    # stake 3 tokens
    assert_command bash -c "quill --pem-file $PEM_LOCATION/identity.pem neuron-stake --amount 3 --name myneur > stake.call"
    assert_file_not_empty stake.call
    SEND_OUTPUT="$(quill send stake.call --yes)"
    assert_command echo "$SEND_OUTPUT" # replay the output so string matches work
    echo "$SEND_OUTPUT"
    assert_string_match "Method name: claim_or_refresh_neuron_from_account"
    NEURON_ID=`echo "$SEND_OUTPUT" | grep -E 'NeuronId' | grep -Eo '\d{1,3}(_\d{3})+' | sed 's/_//g'`
    echo "NEURON: $NEURON_ID"
    assert_string_match "record { result = opt variant { NeuronId = record { id =" #fragment of a correct response

    # check that staking worked using get-neuron-info
    assert_command bash -c "quill get-neuron-info $NEURON_ID --yes"
    assert_string_match 'stake_e8s = 300_000_000'

    # increase dissolve delay by 6 months
    assert_command bash -c "quill --pem-file $PEM_LOCATION/identity.pem neuron-manage --additional-dissolve-delay-seconds 15778800 $NEURON_ID > more-delay.call"
    assert_file_not_empty more-delay.call
    assert_command quill send more-delay.call --yes #provides no interesting output on succes. Command not failing is good enough here

    # check that increasing dissolve delay worked, this time using list-neurons
    assert_command bash -c "quill --pem-file $PEM_LOCATION/identity.pem list-neurons > neuron.call"
    assert_command quill send neuron.call --yes
    assert_string_match "dissolve_delay_seconds = 15_778_800"
}
