# Authentication Service Functionality

User's can be uniquely identified by their `uuid`.

In our case, the Authentication service must provide bindings between the UUID and Client's signature key pairs.

The first step is to provide a binding between Users, an application level concept, with Clients, an MLS concept.

This is accomplished using the `identity` field in the User table and the `signature` field in the Client table. The User sign's the Client's signature key with their identity key.

Next, we must provide a binding between a User's `identity` and their `uuid`.

This binding can be performed in two different ways each with their own security implications depending on the needed security properties.

The strongest method of binding is out of band in person confirmation of identity keys.

The weaker method of binding is a TOFU API request.

If we would like to provide a binding between a User's new `identity` and their former `identity`, we can provide a signature of the new `identity` by the former `identity`.

Authentication Service Steps 2 & 3:
 
Bindings:

Signature Key Pair <-- Signature --> User `identity` <-- TOFU/Out of Band --> UUID (reference identifier)





