pub const ARMS: &[(&str, Arm, &[&str])] = &[
    ("--emit-env-probe", Arm::Emit(env_probe::emit, Grammar::Flags(&[])), &[]),
    ("--run", Arm::Run(run), &[]),
];
