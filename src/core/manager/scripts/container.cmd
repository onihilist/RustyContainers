@echo off
setlocal

call :func "%~1" "%~2"
exit /b

:func
set "action=%~1"
set "container_id=%~2"

if "%action%"=="stop" (
    echo Stopping container: %container_id%
    docker-compose stop "%container_id%"
    if errorlevel 1 (
        echo Failed to stop container: %container_id%
    )
) else if "%action%"=="build" (
      echo Starting container: %container_id%
      docker-compose build "%container_id%"
      if errorlevel 1 (
          echo Failed to build container: %container_id%
      )
) else if "%action%"=="start" (
    echo Starting container: %container_id%
    docker-compose up "%container_id%"
    if errorlevel 1 (
        echo Failed to start container: %container_id%
    )
) else if "%action%"=="pause" (
    echo Pausing container: %container_id%
    docker-compose pause "%container_id%"
    if errorlevel 1 (
        echo Failed to pause container: %container_id%
    )
) else if "%action%"=="resume" (
    echo Resuming container: %container_id%
    docker-compose unpause "%container_id%"
    if errorlevel 1 (
        echo Failed to resume container: %container_id%
    )
) else if "%action%"=="discard" (
    echo Discarding container: %container_id%
    docker-compose rm -f "%container_id%"
    if errorlevel 1 (
        echo Failed to discard container: %container_id%
    )
) else (
    echo Invalid action: "%action%". Please use actions in core::manager::loader::RCAction enum.
)
exit /b