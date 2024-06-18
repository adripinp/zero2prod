https://docs.github.com/es/authentication/connecting-to-github-with-ssh/generating-a-new-ssh-key-and-adding-it-to-the-ssh-agent

#Validate repos remote
git remote -v

git status

#Add ssh key Ubuntu
ssh-keygen -t rsa -b 4096 -C "xxxxxx@xxmail.com"
eval "$(ssh-agent -s)"
ssh-add ~/.ssh/id_rsa
#Add in key github
cat ~/.ssh/id_rsa.pub
