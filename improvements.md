# Sécurité logicielle haut niveau
KING est une application simplifiée de gestion de notes d'étudiants. Elle illustre la mise en place des bonnes pratiques de sécurité logicielle haut niveau, comme le chiffrement des données sensibles, le hachage des mots de passe et l'utilisation d'un sel ou encore l'utilisation d'un système de contrôle d'accès. Ci-dessous, la liste des problèmes à résoudre montre ce qui a été fait pour rendre KING plus sûr, à partir d'un code source ne respectant pas les bonnes pratiques.

# Problèmes à résoudre

1. Mots de passe en clair dans le code
>Les identifiants avec mot de passe étaient codés en dur dans le code.
J'ai créé une base de donnée dédiée aux utilisateurs. Elle pourra aussi acceuillir les étudiants. 
J'en ai profité pour créé un type User qui contient le rôle d'utilisateur afin de préparer le terrain au contrôle d'accès.

2. Les mots de passes ne sont pas hachés.
> J'ai introduit le hachage des mots de passe avec Argon2 afin de protéger les mots de passe des utilisateurs, y compris s'ils sont faibles. 
Le module utilisé ajoute automatiquement un sel.

3. Absence de contrôle d'accès. Un étudiant peu se faire passer pour un autre afin de voir ses notes.
> Un système de rôle et un contrôle d'accès avec Casbin a été mis en place.

4. Lors d'un échec, le mot de passe est imprimé dans les logs.
> J'ai supprimé cette information des logs.

5. Absence de log d'accès
> Nous avons ajouté des logs de niveau info lors des authentifications réussies.

6. Des `println!` sont utilisés pour afficher des information de debugage.
> Le module de log était déjà en place (terminal logger). J'ai simplement remplacé ces `println!` par des `trace!`.

7. Pas de logs pour les erreurs où les tentatives d'accès non autorisées.
> Ces informations on été ajoutées au moyen de `error!` ou `warn!`.

8. Les notes sont des données sensibles. Elles ne sont pas traitées en conséquence.
> Un chiffrement des notes a été mis en place. A noter cependant que la solution actuelle n'est pas idéale et sert surtout de POC. Un réflexion sur la gestion des clefs est encore à mener.

9.  Des `panic!` sont présents dans le code.
> Ils ont été retirés au profit d'une gestion plus fine des erreurs.

10. Le entrées utilisateurs ne snt pas vérifiées. 
> Une restriction des caractères à disposition a été introduite sur les noms d'utilisateur. La taille du mot de passe et du nom d'utilisateur ont désormais une taille maximale.
