FROM public.ecr.aws/nginx/nginx:mainline
EXPOSE 80
COPY index.html /usr/share/nginx/html
