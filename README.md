# rustspyder
A web crawling robot, written in Rust

## configuration
Ideally, all configuration for the robot will be performed via configuration files. The unique parameters to provide to binary should be:

1. verbosity
2. help
3. configuration file option

Preferably, the options to binary will be provided in both short and long options (-h|--help; -v|--verbose; -c|--configuration-file).

## Web parsing and navigation
Initially, rustspyder is thought to be used with "plain html" web pages. Navigating heavy javascripted sites is not the use of case, although ideally in later phases should be handled similarly to plan html sites. Robot will perform easy mechanisms to navigate through different pages by configuration provided, so that crawler will be able to navigate 

## Filters
Web crawler will provide filters so that once a web page is retrieved, a filter can be provided to keep some of the information in the site that could be useful, such as a model, a price, a description, etc. A set of common filters (for price parsing, html elimination, model or description parsing, etc) will be available for user.

## Concepts
Crawler will provide a mechanism to store "concepts" associated to filters. Normally, a concept will match a filter. "Price" concept with "filter" regexp
would be saved so that concept can be later used (stored in database, stored in file, displayed in terminal, whatever)

## Dumps
Dumps will be mechanisms to dump each of the concept, together with its value, to a particular output artifact (file, database, whatever)

As stated initially, all this should be handled via one or several configuration files (it will depend on design requirements)
