cmake_minimum_required(VERSION 3.25)

# This is your project statement. You should always list languages;
# Listing the version is nice here since it sets lots of useful variables
project(
  epitome
  VERSION 1.0
  LANGUAGES C)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# Adding something we can run - Output name matches target name
add_library(epitome SHARED src/wrapper.c)
target_include_directories(epitome PRIVATE target)
message("Building ${CMAKE_BUILD_TYPE}")
IF(CMAKE_BUILD_TYPE MATCHES Debug)
  execute_process(COMMAND cargo build WORKING_DIRECTORY ${CMAKE_SOURCE_DIR})
  message("Linking debug version of implementation")
  target_link_directories(epitome PRIVATE target/debug)
else()
  execute_process(COMMAND cargo build --release WORKING_DIRECTORY ${CMAKE_SOURCE_DIR})
  message("Linking release version of implementation")
  target_link_directories(epitome PRIVATE target/release)
ENDIF()

# Make sure you link your targets with this command. It can also link libraries and
# even flags, so linking a target that does not exist will not give a configure-time error.
target_link_libraries(epitome PRIVATE epitome_impl)