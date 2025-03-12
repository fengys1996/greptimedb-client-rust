cmake_minimum_required(VERSION 3.20)
project(a-abi-demo)

set(CMAKE_C_STANDARD 99)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_executable(main
	${PROJECT_SOURCE_DIR}/main.c
)
target_link_directories(main PUBLIC
			  "${PROJECT_SOURCE_DIR}/lib"
			  )
target_include_directories(main PUBLIC
                          "${PROJECT_SOURCE_DIR}/include"
                          )
target_link_libraries(main greptimedb_ingester)
