# Assign and loop

{% assign kits = "gate-sdk,site-kit" | split: "," %}
{% for kit in kits %}- {{ kit }}
{% endfor %}
