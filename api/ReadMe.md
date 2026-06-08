# API

## sqlx & slqx-cli

Librairie pour communiquer avec bdd. Requetes verifiees a la compilation et pool partagee pour la connexion (plusieurs connexions et pas une connexion par requete qui devient vite couteux en energie). Migrations integrees

### Commandes a connaitres

`sqlx migrate add -r create_users_table` -> Creation fichier migration avec le up et down possible
`sqlx migrate run` -> Applique la migration en db
`sqlx migrate revert` -> faire fonctionner le fichier .down qui va supprimer la table
