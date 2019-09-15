cmake_minimum_required(VERSION 3.1.0)

project(med)

find_package(Qt5 COMPONENTS Widgets REQUIRED)

include_directories(.)

set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Wextra")

add_executable(med
  src/exec.cc
  src/file_descriptor.cc
  src/main.cc
  src/shell.cc)

set_target_properties(med PROPERTIES AUTOMOC ON AUTOUIC ON)

target_link_libraries(med Qt5::Widgets)
