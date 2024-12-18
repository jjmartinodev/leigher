cmake_minimum_required(VERSION 3.12)

project(leigher)

find_package(OpenGL REQUIRED)

include_directories(dependencies/glfw/include)
include_directories(dependencies/glew/include)

add_subdirectory(dependencies/glfw)
add_subdirectory(dependencies/glew/build/cmake)

add_executable(
    ${PROJECT_NAME}
    src/main.cpp
)

target_link_libraries(
    ${PROJECT_NAME}
    PRIVATE
    glfw
    glew_s
    ${OPENGL_LIBRARIES}
)