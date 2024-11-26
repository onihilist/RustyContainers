@echo off
setlocal

call :func "%1" "%2"

:func
set "action=%~1"
set "container_id=%~2"

if "%action%"=="stop" (
    docker stop %container_id%
) else if "%action%"=="start" (
    docker start %container_id%
) else if "%action%"=="pause" (
    docker pause %container_id%
) else if "%action%"=="resume" (
    docker unpause %container_id%
) else if "%action%"=="discard" (
    docker rm %container_id%
) else (
    echo "Invalid case. Please use actions in core::manager::loader::RCAction enum."
)
exit /b
