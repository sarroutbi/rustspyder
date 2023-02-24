# rustspyder
A web crawling robot, written in Rust

# configuration
Ideally, all configuration for the robot will be performed via configuration files. The unique parameters to provide to binary should be:
1 - verbosity
2 - help
3 - configuration file option
Preferably, the options to binary will be provided in both short and long options (-h|--help; -v|--verbose; -c|--configuration-file).
