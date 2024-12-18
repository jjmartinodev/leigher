#include <GL/glew.h>
#include <GLFW/glfw3.h>

int main(void) {
    glfwInit();
    GLFWwindow* window = glfwCreateWindow(800,600,"hii",nullptr, nullptr);
    glfwMakeContextCurrent(window);
    glewInit();
    while(!glfwWindowShouldClose(window)) {
        glfwPollEvents();
        glfwSwapBuffers(window);
    }
    glfwDestroyWindow(window);
    glfwTerminate();
    
    return 0;
}