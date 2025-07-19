# Example highlight queries for Dockerfile

; Instructions
"FROM" @keyword
"MAINTAINER" @keyword
"RUN" @keyword
"CMD" @keyword
"LABEL" @keyword
"EXPOSE" @keyword
"ENV" @keyword
"ADD" @keyword
"COPY" @keyword
"ENTRYPOINT" @keyword
"VOLUME" @keyword
"USER" @keyword
"WORKDIR" @keyword
"ARG" @keyword
"ONBUILD" @keyword
"STOPSIGNAL" @keyword
"HEALTHCHECK" @keyword
"SHELL" @keyword

; Options
"--from" @property
"--chown" @property
"--chmod" @property

; Strings
(string) @string
(quoted_string) @string

; Comments
(comment) @comment

; Variables
(expansion) @variable

; Punctuation
"[" @punctuation.bracket
"]" @punctuation.bracket
"=" @operator