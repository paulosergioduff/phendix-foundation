package com.example.customcomponents;

import com.mendix.systemwideinterfaces.core.UserAction;

public class CustomComponent implements UserAction<Void> {

    private String message;

    public CustomComponent(String message) {
        this.message = message;
    }

    @Override
    public Void executeAction() throws Exception {
        System.out.println("Mensagem do componente personalizado: " + message);
        // Aqui você pode adicionar lógica adicional, como integração com APIs externas, manipulação de dados, etc.
        return null;
    }
}
